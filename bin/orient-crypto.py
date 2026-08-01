#!/usr/bin/env python3
"""Local secrets + HMAC/AES helpers for the orientation pass store.

Not a fortress: keys live on disk next to the scripts. Goal is mild friction
against casual `ls` / hand-editing, not protection from someone who reads
this file.

Secrets file (gitignored, auto-created):
  bin/orient-secrets   JSON {version, hmac_key, aes_key, created_ts}

Pass file:
  .orient/passes/<hmac_filename>.json
  envelope {v, nonce, ciphertext}  — AES-256-GCM of signed pass body
"""
from __future__ import annotations

import hashlib
import hmac
import json
import os
import secrets
import sys
import time
from pathlib import Path

try:
    from cryptography.hazmat.primitives.ciphers.aead import AESGCM
except ImportError as e:  # pragma: no cover
    print(
        "orient-crypto: needs the 'cryptography' package "
        f"(AES-GCM). Import failed: {e}",
        file=sys.stderr,
    )
    sys.exit(3)

ROOT = Path(__file__).resolve().parent.parent
SECRETS_PATH = ROOT / "bin" / "orient-secrets"
PASSES_DIR = ROOT / ".orient" / "passes"

# Domain separation for HMAC uses (so filename tag ≠ body tag).
_HMAC_NAME = b"vivarium-orient-pass-name-v1|"
_HMAC_BODY = b"vivarium-orient-pass-body-v1|"


def load_or_create_secrets() -> dict:
    if SECRETS_PATH.is_file():
        try:
            data = json.loads(SECRETS_PATH.read_text(encoding="utf-8"))
        except Exception as e:
            raise SystemExit(f"orient-crypto: unreadable {SECRETS_PATH}: {e}") from e
        if not data.get("hmac_key") or not data.get("aes_key"):
            raise SystemExit(f"orient-crypto: {SECRETS_PATH} missing hmac_key/aes_key")
        return data

    SECRETS_PATH.parent.mkdir(parents=True, exist_ok=True)
    data = {
        "version": 1,
        "hmac_key": secrets.token_hex(32),  # 256-bit
        "aes_key": secrets.token_hex(32),  # AES-256
        "created_ts": int(time.time()),
        "note": "auto-created; gitignored; delete to mint new keys (invalidates all passes)",
    }
    # Restrictive perms when possible
    fd = os.open(
        SECRETS_PATH,
        os.O_WRONLY | os.O_CREAT | os.O_EXCL,
        0o600,
    )
    with os.fdopen(fd, "w", encoding="utf-8") as f:
        json.dump(data, f, indent=2)
        f.write("\n")
    return data


def _hmac_key_bytes(sec: dict) -> bytes:
    return bytes.fromhex(sec["hmac_key"])


def _aes_key_bytes(sec: dict) -> bytes:
    return bytes.fromhex(sec["aes_key"])


def pass_filename_stem(session_id: str, sec: dict | None = None) -> str:
    """HMAC-SHA256 hex of session_id (keyed) — not reversible without the key."""
    sec = sec or load_or_create_secrets()
    dig = hmac.new(
        _hmac_key_bytes(sec),
        _HMAC_NAME + session_id.encode("utf-8"),
        hashlib.sha256,
    ).hexdigest()
    return dig


def pass_path_for(session_id: str, root: Path | None = None) -> Path:
    root = root or ROOT
    sec = load_or_create_secrets()
    return root / ".orient" / "passes" / f"{pass_filename_stem(session_id, sec)}.json"


def _canonical_body(pass_doc: dict) -> bytes:
    """Stable bytes for HMAC: sorted keys, no mac field, compact JSON."""
    body = {k: v for k, v in pass_doc.items() if k != "mac"}
    return json.dumps(body, sort_keys=True, separators=(",", ":")).encode("utf-8")


def sign_pass_doc(pass_doc: dict, sec: dict | None = None) -> dict:
    """Return a copy of pass_doc with mac = HMAC-SHA256 hex of canonical body."""
    sec = sec or load_or_create_secrets()
    out = dict(pass_doc)
    out.pop("mac", None)
    dig = hmac.new(
        _hmac_key_bytes(sec),
        _HMAC_BODY + _canonical_body(out),
        hashlib.sha256,
    ).hexdigest()
    out["mac"] = dig
    return out


def verify_pass_mac(pass_doc: dict, sec: dict | None = None) -> bool:
    sec = sec or load_or_create_secrets()
    got = pass_doc.get("mac")
    if not got or not isinstance(got, str):
        return False
    expect = hmac.new(
        _hmac_key_bytes(sec),
        _HMAC_BODY + _canonical_body(pass_doc),
        hashlib.sha256,
    ).hexdigest()
    return hmac.compare_digest(got, expect)


def write_pass(session_id: str, pass_doc: dict, root: Path | None = None) -> Path:
    """Sign + AES-GCM encrypt pass_doc → envelope JSON at pass_path_for(session_id)."""
    root = root or ROOT
    sec = load_or_create_secrets()
    signed = sign_pass_doc(pass_doc, sec)
    plaintext = json.dumps(signed, indent=2).encode("utf-8") + b"\n"

    nonce = secrets.token_bytes(12)  # GCM standard
    aesgcm = AESGCM(_aes_key_bytes(sec))
    ct = aesgcm.encrypt(nonce, plaintext, associated_data=session_id.encode("utf-8"))

    envelope = {
        "v": 7,
        "alg": "AES-256-GCM",
        "nonce": nonce.hex(),
        "ciphertext": ct.hex(),
        # session_id is NOT stored in the clear envelope; AAD binds ciphertext to id
    }
    path = pass_path_for(session_id, root)
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(envelope, indent=2) + "\n", encoding="utf-8")
    try:
        path.chmod(0o600)
    except OSError:
        pass
    return path


def read_pass(session_id: str, root: Path | None = None) -> dict:
    """Decrypt + verify MAC; raise ValueError on failure."""
    root = root or ROOT
    sec = load_or_create_secrets()
    path = pass_path_for(session_id, root)
    if not path.is_file():
        raise FileNotFoundError(str(path))

    env = json.loads(path.read_text(encoding="utf-8"))
    if env.get("v") != 7 or "nonce" not in env or "ciphertext" not in env:
        raise ValueError("pass envelope is not v7 AES-GCM")

    aesgcm = AESGCM(_aes_key_bytes(sec))
    try:
        plaintext = aesgcm.decrypt(
            bytes.fromhex(env["nonce"]),
            bytes.fromhex(env["ciphertext"]),
            associated_data=session_id.encode("utf-8"),
        )
    except Exception as e:
        raise ValueError(f"AES-GCM decrypt failed (wrong key or tampered): {e}") from e

    doc = json.loads(plaintext.decode("utf-8"))
    if not verify_pass_mac(doc, sec):
        raise ValueError("pass MAC mismatch (tampered body or wrong hmac_key)")
    if doc.get("session_id") != session_id:
        raise ValueError("pass session_id does not match token")
    return doc


def main() -> None:
    args = sys.argv[1:]
    if not args or args[0] in ("-h", "--help"):
        print(
            "usage:\n"
            "  bin/orient-crypto.py ensure-secrets   # create bin/orient-secrets if missing\n"
            "  bin/orient-crypto.py pass-path <sid>\n"
            "  bin/orient-crypto.py read <sid>       # decrypt+verify → JSON stdout\n",
            file=sys.stderr,
        )
        sys.exit(0)
    cmd = args[0]
    if cmd == "ensure-secrets":
        s = load_or_create_secrets()
        print(f"orient-crypto: secrets at {SECRETS_PATH} (created_ts={s.get('created_ts')})")
        return
    if cmd == "pass-path" and len(args) >= 2:
        print(pass_path_for(args[1]))
        return
    if cmd == "read" and len(args) >= 2:
        print(json.dumps(read_pass(args[1]), indent=2))
        return
    print(f"orient-crypto: unknown command {cmd}", file=sys.stderr)
    sys.exit(2)


if __name__ == "__main__":
    main()
