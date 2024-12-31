svg_data_uri:
	rm src/svg_part_data_uri/*
	# TODO: --datauri unenc once https://github.com/svg/svgo/pull/2053 merges
	svgo --input src/svg_part/*.svg -p 3 --multipass --pretty --output src/svg_part_data_uri/
	cat src/svg_part_data_uri/attitude_indicator_pitch.svg | sed 's/width=\"100\"\ height=\"100\"/width=\"100%\"\ height=\"100%\"\ overflow=\"visible\"\ style=\"overflow: visible !important;\"/g' > src/svg_part_data_uri/attitude_indicator_pitch.svg.raw
	rm src/svg_part_data_uri/attitude_indicator_pitch.svg
	$(foreach file, $(wildcard src/svg_part_data_uri/*.svg), printf "data:image/svg+xml,%s" `cat $(file) | urlenc enc | sed 's/+/%20/g'` > $(file);)

deps:
	npm install svgo
	go install github.com/bww/urlencode/cmd/urlenc@latest

fmt:
	cargo fmt
	cargo fmt --manifest-path pages/Cargo.toml