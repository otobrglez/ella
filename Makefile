all: libprom2json/prom2json.so
	echo "Done"

libprom2json/prom2json.so:
	go build \
		-o libprom2json/prom2json.so \
		-buildmode=c-shared \
		libprom2json/main.go

libprom2json/prom2json.a:
	# CGO_ENABLED=1 go build --ldflags '-linkmode external -extldflags=-static'
	CGO_ENABLED=1 go build \
		-v \
		--ldflags '-linkmode external -extldflags=-static' \
		-o libprom2json/prom2json.a \
		-buildmode=c-archive \
		libprom2json/main.go


clean:
	rm -rf libprom2json/*.a libprom2json/*.h libprom2json/*.so libprom2json/bindings.rs

