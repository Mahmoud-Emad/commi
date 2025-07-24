# Security Policy

## Supported Versions

We actively support the following versions of Commi with security updates:

| Version | Supported          |
| ------- | ------------------ |
| 4.0.x   | :white_check_mark: |
| 3.x.x   | :x:                |
| < 3.0   | :x:                |

## Reporting a Vulnerability

We take security vulnerabilities seriously. If you discover a security vulnerability in Commi, please report it responsibly.

### How to Report

**Please do NOT report security vulnerabilities through public GitHub issues.**

Instead, please report security vulnerabilities by emailing: **<security@commi-project.org>**

Include the following information in your report:

- Description of the vulnerability
- Steps to reproduce the issue
- Potential impact
- Any suggested fixes (if available)

### What to Expect

- **Acknowledgment**: We will acknowledge receipt of your vulnerability report within 48 hours.
- **Initial Assessment**: We will provide an initial assessment within 5 business days.
- **Updates**: We will keep you informed of our progress throughout the investigation.
- **Resolution**: We aim to resolve critical vulnerabilities within 30 days.

### Security Best Practices

When using Commi in production:

1. **API Key Security**
   - Store API keys securely using environment variables
   - Never commit API keys to version control
   - Use different API keys for different environments
   - Regularly rotate API keys

2. **Network Security**
   - Ensure HTTPS connections to Google's Gemini API
   - Use firewall rules to restrict outbound connections if needed
   - Monitor network traffic for anomalies

3. **File System Security**
   - Run Commi with minimal required permissions
   - Ensure git repositories are properly secured
   - Validate file paths and prevent directory traversal

4. **Input Validation**
   - Be cautious with commit messages containing sensitive data
   - Review generated commit messages before committing
   - Use the `--cached` flag to limit diff scope

### Security Features

Commi includes several built-in security features:

- **Input Sanitization**: All user inputs are validated and sanitized
- **Secure API Communication**: All API calls use HTTPS with certificate validation
- **Sensitive Data Detection**: Warns when potentially sensitive data is detected in diffs
- **Safe Defaults**: Conservative defaults that prioritize security

### Vulnerability Disclosure Timeline

1. **Day 0**: Vulnerability reported
2. **Day 1-2**: Acknowledgment sent to reporter
3. **Day 3-7**: Initial assessment and triage
4. **Day 8-30**: Development and testing of fix
5. **Day 30**: Public disclosure and release of patched version

### Security Updates

Security updates are released as patch versions (e.g., 4.0.1, 4.0.2) and are clearly marked in the changelog. We recommend:

- Enabling automatic updates where possible
- Subscribing to release notifications
- Regularly checking for updates using `commi update check`

### Contact

For security-related questions or concerns:

- Email: <mahmmoud.hassanein@gmail.com>
- For general questions: [GitHub Discussions](https://github.com/Mahmoud-Emad/commi/discussions)

Thank you for helping keep Commi and our users safe!
