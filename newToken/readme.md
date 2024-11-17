solana-keygen grind --starts-with nix:1
solana config set --url devnet
solana config set --keypair bosHcXdpMNegSZEGzt9bjmzTes1B6KBw6uEoTtGEt5N.json
solana-keygen grind --starts-with mnt:1
spl-token create-token --program-id TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb --enable-metadata mnt7gTPzdrKPrTPb8BgPA3Bra1mvjNkDC45P6x1T4w9.json

create a metadata.json

{
	"name": "Example Token",
	"symbol": "EXMPL",
	"description": "Example token tutorial",
	"image": "https://"
}
