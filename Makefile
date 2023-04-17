install:
	cargo build --release
	mv target/release/mls ~/bin/ls

link:
	ln -s ./ls ~/bin/ll
	ln -s ./ls ~/bin/la
	ln -s ./ls ~/bin/lt
	ln -s ./ls ~/bin/ltl
	ln -s ./ls ~/bin/llt
	ln -s ./ls ~/bin/tree
	ln -s ./ls ~/bin/lla
	ln -s ./ls ~/bin/lal
	ln -s /bin/ls ~/bin/lso
# ln -s /bin/ls ~/bin/lsd

del:
	rm ~/bin/ll
	rm ~/bin/la
	rm ~/bin/lt
	rm ~/bin/ltl
	rm ~/bin/llt
	rm ~/bin/tree
	rm ~/bin/lla
	rm ~/bin/lal
	rm ~/bin/lso
# rm ~/bin/lsd