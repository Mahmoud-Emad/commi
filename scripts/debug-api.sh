#!/bin/sh

# Debug script to test GitHub API response

REPO="Mahmoud-Emad/commi"
GITHUB_API="https://api.github.com/repos/$REPO/releases/latest"

echo "Testing GitHub API response..."
echo "API URL: $GITHUB_API"
echo ""

# Test with curl
if command -v curl >/dev/null 2>&1; then
    echo "=== Testing with curl ==="
    tmpfile=$(mktemp)
    
    echo "Fetching API response..."
    if curl -sSf "$GITHUB_API" > "$tmpfile" 2>/dev/null; then
        echo "✅ API call successful"
        echo "Response size: $(wc -c < "$tmpfile") bytes"
        echo ""
        
        # Test jq parsing
        if command -v jq >/dev/null 2>&1; then
            echo "=== Available assets (jq) ==="
            jq -r '.assets[].name' "$tmpfile" 2>/dev/null || echo "❌ jq parsing failed"
            echo ""
            
            # Test specific asset lookup
            asset_name="commi-x86_64-unknown-linux-gnu"
            echo "=== Looking for: $asset_name ==="
            url=$(jq -r ".assets[] | select(.name == \"$asset_name\") | .browser_download_url" "$tmpfile" 2>/dev/null)
            if [ -n "$url" ]; then
                echo "✅ Found: $url"
            else
                echo "❌ Not found"
            fi
        else
            echo "jq not available, testing grep fallback..."
            echo "=== Available assets (grep) ==="
            grep '"name":' "$tmpfile" | sed 's/.*"name": *"\([^"]*\)".*/\1/' || echo "❌ grep parsing failed"
        fi
        
        echo ""
        echo "=== First 500 chars of response ==="
        head -c 500 "$tmpfile"
        echo ""
        echo "..."
        
    else
        echo "❌ API call failed"
    fi
    
    rm -f "$tmpfile"
else
    echo "curl not available"
fi

echo ""
echo "=== System info ==="
echo "OS: $(uname -s)"
echo "Arch: $(uname -m)"
echo "Available tools:"
command -v curl >/dev/null && echo "  ✅ curl" || echo "  ❌ curl"
command -v wget >/dev/null && echo "  ✅ wget" || echo "  ❌ wget"
command -v jq >/dev/null && echo "  ✅ jq" || echo "  ❌ jq"
command -v awk >/dev/null && echo "  ✅ awk" || echo "  ❌ awk"
