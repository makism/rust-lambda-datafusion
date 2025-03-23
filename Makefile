LAMBDA_NAME = lambda-datafusion

clean:
	rm -fv target/

build:
	cargo lambda build --release --arm64 --output-format zip
