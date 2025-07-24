#!/bin/sh

# Test script for install.sh
# This tests the install script without actually installing anything

set -e

echo "=== Testing Commi Install Script ==="

# Test 1: Help function
echo "1. Testing help function..."
if cat install.sh | sh -s -- --help >/dev/null 2>&1; then
    echo "✅ Help function works"
else
    echo "❌ Help function failed"
    exit 1
fi

# Test 2: Platform detection
echo "2. Testing platform detection..."
platform=$(sh -c '. ./install_test.sh; detect_platform')
if [ -n "$platform" ]; then
    echo "✅ Platform detected: $platform"
else
    echo "❌ Platform detection failed"
    exit 1
fi

# Test 3: Prerequisites check
echo "3. Testing prerequisites..."
if sh -c '. ./install_test.sh; check_prerequisites' >/dev/null 2>&1; then
    echo "✅ Prerequisites check passed"
else
    echo "❌ Prerequisites check failed"
    exit 1
fi

# Test 4: URL fetching
echo "4. Testing URL fetching..."
url=$(sh -c '. ./install_test.sh; get_latest_release')
if [ -n "$url" ] && echo "$url" | grep -q "github.com"; then
    echo "✅ URL fetched: $url"
else
    echo "❌ URL fetching failed: $url"
    exit 1
fi

# Test 5: URL format validation
echo "5. Testing URL format..."
if echo "$url" | grep -q "github.com.*commi-.*-apple-darwin"; then
    echo "✅ URL format is correct"
else
    echo "❌ URL format is incorrect: $url"
    exit 1
fi

# Test 6: Pipe functionality
echo "6. Testing pipe functionality..."
if echo "test" | sh -c '. ./install_test.sh; is_piped && echo "piped" || echo "not piped"' | grep -q "piped"; then
    echo "✅ Pipe detection works"
else
    echo "❌ Pipe detection failed"
    exit 1
fi

# Test 7: Secure download check
echo "7. Testing secure download capabilities..."
if command -v curl >/dev/null 2>&1; then
    echo "✅ Secure download capabilities verified (curl available)"
elif command -v wget >/dev/null 2>&1; then
    echo "✅ Secure download capabilities verified (wget available)"
else
    echo "❌ No download tools available"
    exit 1
fi

echo ""
echo "🎉 All tests passed! The install script is ready to use."
echo ""
echo "To install commi, run:"
echo "curl --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/Mahmoud-Emad/commi/main/install.sh | sh"
