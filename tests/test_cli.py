import subprocess

def test_help():
    result = subprocess.run(['commi', '--help'], capture_output=True, text=True)
    assert 'usage: commi' in result.stdout  # Ensure the usage info is in the output
    assert 'Welcome to XCommi' not in result.stdout  # Banner should not appear in --help

# def test_cached_flag():
#     result = subprocess.run(['commi', '--cached'], capture_output=True, text=True)
#     assert 'Generating commit message from staged changes' in result.stdout

# def test_copy_flag():
#     result = subprocess.run(['commi', '--copy'], capture_output=True, text=True)
#     assert 'The commit message has been copied to your clipboard!' in result.stdout
