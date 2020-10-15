#!/usr/bin/bash
cd pkgs;
for dir in $(ls); do
	if [[ "$dir" != "." ]] \
	&& [[ "$dir" != ".." ]] \
	&& [[ "$1" == "$dir" ]]; then
		cd "$dir";
		makepkg -isf --noconfirm;
		cd "..";
	fi;
done;
