build:
	cargo build
run:
	cat input.json | cargo run

build-release:
	cargo build --release

run-release:
	cat input.json | ./target/release/grafana-resource

docker:
	docker build -t teliaoss/grafana-resource .

run-docker:
	cat input.json | docker run -i --rm --entrypoint /opt/resource/out teliaoss/grafana-resource