
# download openapi.yaml at https://raw.githubusercontent.com/igomez10/microservices/mainline/socialapp/openapi.yaml
download-openapi:
	curl https://raw.githubusercontent.com/igomez10/microservices/mainline/socialapp/openapi.yaml -o openapi.yaml

generate-rust-client:
	# delete old generated code
	rm -rf ./rust-sdk
	
	# generate new code
	openapi-generator generate -i openapi.yaml  \
	-g rust \
	-o ./rust-sdk  \
	-p packageName=rust-sdk \
	--git-user-id=igomez10 \
	--git-repo-id=rust-server \
	--additional-properties=library=reqwest

	# GENERATED CODE SUCCESSFULLY

generate-rust-server:
	# delete old generated code
	rm -rf ./rust-server
	
	# generate new code
	openapi-generator generate -i openapi.yaml  \
	-g rust-server \
	-o ./rust-server  \
	-p packageName=rust-server \
	--git-user-id=igomez10 \
	--git-repo-id=rust-server

	# GENERATED CODE SUCCESSFULLY
