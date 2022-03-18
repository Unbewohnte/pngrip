cross: clean
	cargo build --release --target x86_64-pc-windows-gnu
	cargo build --release --target x86_64-unknown-linux-musl
	cargo build --release --target i686-unknown-linux-musl
	cargo build --release --target i686-pc-windows-gnu

	mkdir -p release/pngrip_x86_64-pc-windows-gnu
	mkdir -p release/pngrip_x86_64-unknown-linux-musl
	mkdir -p release/pngrip_i686-unknown-linux-musl
	mkdir -p release/pngrip_i686-pc-windows-gnu

	cp LICENSE release/pngrip_x86_64-pc-windows-gnu
	cp LICENSE release/pngrip_x86_64-unknown-linux-musl
	cp LICENSE release/pngrip_i686-unknown-linux-musl
	cp LICENSE release/pngrip_i686-pc-windows-gnu

	cp target/x86_64-pc-windows-gnu/release/pngrip.exe release/pngrip_x86_64-pc-windows-gnu
	cp target/x86_64-unknown-linux-musl/release/pngrip release/pngrip_x86_64-unknown-linux-musl
	cp target/i686-unknown-linux-musl/release/pngrip release/pngrip_i686-unknown-linux-musl
	cp target/i686-pc-windows-gnu/release/pngrip.exe release/pngrip_i686-pc-windows-gnu

	cd release && \
	zip -r pngrip_x86_64-pc-windows-gnu pngrip_x86_64-pc-windows-gnu/ && \
	zip -r pngrip_x86_64-unknown-linux-musl pngrip_x86_64-unknown-linux-musl && \
	zip -r pngrip_i686-unknown-linux-musl pngrip_i686-unknown-linux-musl && \
	zip -r pngrip_i686-pc-windows-gnu pngrip_i686-pc-windows-gnu


clean:
	rm -rf release