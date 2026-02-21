#!/usr/bin/env bash
set -euo pipefail

fail_count=0
warn_count=0

ok() {
  printf "OK   %s\n" "$1"
}

warn() {
  printf "WARN %s\n" "$1"
  warn_count=$((warn_count + 1))
}

fail() {
  printf "FAIL %s\n" "$1"
  fail_count=$((fail_count + 1))
}

has_cmd() {
  command -v "$1" >/dev/null 2>&1
}

echo "Checking CrabCord build prerequisites..."
echo

if [[ "$(uname -s)" != "Darwin" ]]; then
  fail "macOS is required for this repository right now."
else
  ok "Running on macOS"
fi

if has_cmd rustc && has_cmd cargo; then
  rustc_version="$(rustc --version)"
  cargo_version="$(cargo --version)"
  ok "Rust toolchain detected: ${rustc_version} / ${cargo_version}"
  if [[ "${rustc_version}" == *nightly* ]]; then
    warn "Nightly Rust detected. GPUI docs recommend latest stable Rust."
  fi
else
  fail "Rust toolchain not found. Install via rustup: https://rustup.rs"
fi

if has_cmd xcode-select; then
  if xcode_path="$(xcode-select -p 2>/dev/null)"; then
    ok "Xcode developer dir: ${xcode_path}"
  else
    fail "Xcode command line tools not configured. Run: xcode-select --install"
  fi
else
  fail "xcode-select not found. Install Xcode and command line tools."
fi

if has_cmd xcodebuild; then
  if xcodebuild -version >/dev/null 2>&1; then
    ok "xcodebuild is available"
  else
    fail "xcodebuild is not functional. Open Xcode once and accept the license."
  fi
else
  fail "xcodebuild not found. Install full Xcode app."
fi

if has_cmd xcrun; then
  if metal_path="$(xcrun --find metal 2>/dev/null)"; then
    ok "Metal tool found: ${metal_path}"
  else
    fail "Metal compiler not found. Ensure xcode-select points to full Xcode."
  fi
else
  fail "xcrun not found. Install Xcode command line tools."
fi

if has_cmd cmake; then
  ok "cmake detected: $(cmake --version | head -n 1)"
else
  fail "cmake not found. Install with: brew install cmake"
fi

if has_cmd git; then
  ok "git detected: $(git --version)"
else
  fail "git not found (required for clone and git-based dependencies)."
fi

if has_cmd curl; then
  if curl -IfsS --max-time 5 https://github.com >/dev/null; then
    ok "Network check passed (github.com reachable)"
  else
    warn "Network check failed. First build may fail while fetching dependencies."
  fi
else
  warn "curl not found. Skipping network check."
fi

echo
if [[ -f Cargo.lock ]]; then
  ok "Cargo.lock found (reproducible dependency resolution is enabled)"
else
  warn "Cargo.lock missing. Builds may vary across machines."
fi

echo
if [[ "$fail_count" -gt 0 ]]; then
  echo "Prereq check failed with ${fail_count} issue(s) and ${warn_count} warning(s)."
  exit 1
fi

echo "Prereq check passed with ${warn_count} warning(s)."
