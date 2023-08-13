# env/bin/bash
base_file_name="test"
content=$1
match="documentclass{article}"
replace="documentclass[border=2pt,varwidth]{standalone}"
content=${content/"$match"/"$replace"}
content="%${content#*%}"
echo "$content" >> $base_file_name.tex
