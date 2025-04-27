# List all recipes
a0:
	just -l

# Get all encrypted messages in the room
get-e2e-room-messages:
	echo "Retrieving encrypted messages from server and saving it to CSV file"
	hurl loadHistory.hurl | jq -r '.message' | jq -r '["name", "message"], (.result.messages[] | [.u.name, .content.ciphertext]) | @csv' > messages.csv

# Decode messages
decode-e2e-room-messages: 
	echo "Decoding room key and messages"
	cargo run rsa_private_key.pem session_encoded.key messages.csv

# Use your own data for authorization
get-config-from-template:
	echo "Making hurl config - loadHistory.hurl"
	cat loadHistory.hurl.tpl | sed -e 's/%%server%%/rocketchat.com/;s/%%user%%/username/;s/%%password%%/password/;s/%%roomid%%/kFNnZ57tHiPgxtmS2dAFAfjsmimLpQB/' > loadHistory.hurl
