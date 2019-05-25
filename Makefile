build:
	cargo build
run:
	cat input.json | cargo run --bin ${target}

build-release:
	cargo build --release

run-release:
	cat input.json | ./target/release/grafana-resource

docker:
	docker build -t teliaoss/grafana-resource:dev .

run-docker:
	cat input.json | docker run -i --rm --entrypoint /opt/resource/out teliaoss/grafana-resource:dev