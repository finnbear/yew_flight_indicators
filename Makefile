svg_data_uri:
	# TODO: --datauri unenc once https://github.com/svg/svgo/pull/2053 merges
	svgo --input svg/* -p 1 --multipass --output src/svg_data_uri/
	$(foreach file, $(wildcard src/svg_data_uri/*), printf "data:image/svg+xml,%s" `cat $(file) | urlenc enc | sed 's/+/%20/g'` > $(file);)

deps:
	npm install svgo
	go install github.com/bww/urlencode/cmd/urlenc@latest

fmt:
	cargo fmt
	cargo fmt --manifest-path pages/Cargo.toml