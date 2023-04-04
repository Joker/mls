install:
	cargo build --release
	mv target/release/mls ~/bin/ls

link:
	ln -s ./ls ~/bin/ll
	ln -s ./ls ~/bin/la
	ln -s ./ls ~/bin/lt
	ln -s ./ls ~/bin/lla
	ln -s ./ls ~/bin/lal
	ln -s ./ls ~/bin/lsd

del:
	rm ~/bin/ll ~/bin/la ~/bin/lt ~/bin/lla ~/bin/lal ~/bin/lsd