CARGO=cargo

run: .env
	set -o allexport && . ./$< && $(CARGO) run 
