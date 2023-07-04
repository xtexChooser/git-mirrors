common/src/main/resources/data/quaedam/projected-person-names:
	curl -Ls https://github.com/wainshine/Chinese-Names-Corpus/raw/master/English_Names_Corpus/English_Names_Corpus%EF%BC%882W%EF%BC%89.txt | tail -n +4 | shuf | head -n 5000 > $@.tmp
	#curl -Ls https://github.com/wainshine/Chinese-Names-Corpus/raw/master/Chinese_Names_Corpus/Chinese_Names_Corpus%EF%BC%88120W%EF%BC%89.txt | tail -n +4 | shuf | head -n 2500 >> $@.tmp
	cat $@.tmp | tr -d '\015' > $@
	rm $@.tmp

.PHONY: common/src/main/resources/data/quaedam/projected-person-names
