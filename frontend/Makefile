%:
	@echo "Unknown target '$@' skipping"
create:
	npx create-next-app@latest . --ts --eslint --app --use-npm --yes
generate:
	npm install
	cp -rf ../openapi out
run:
	npm run dev
deploy:
	npm run build
clean:
	npm cache clean --force
