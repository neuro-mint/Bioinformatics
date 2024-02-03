from Bio.Seq import Seq
from Bio.SeqUtils import gc_fraction
from Bio import motifs
from Bio import ExPASy
from Bio import SwissProt

DNA_seq = Seq('ATTTCGTCGGTTCCTGGAATTACACGCGCGTCAGATTCTCGGCCCATGCAT')

#getting compliment of the given sequencce
compliment = DNA_seq.complement()
print('Complement -> ', compliment)

#reverwse complement
rev_complement = DNA_seq.reverse_complement()
print('Reverse complement -> ', rev_complement)

#translating a sequence
translated = DNA_seq.translate()
print('Translated sequence -> ', translated)

#gives counts of of individual nucleotides in the seq
def seq_info(seq):
	n_dict = {'A':0, 'T':0, 'C':0, 'G':0}
	for i in seq:
		n_dict[i] +=1
	return n_dict
print('Individual nucleotide count : ', seq_info(DNA_seq))

#GC count
print('GC countent : ', gc_fraction(DNA_seq)*100, '%')

#getting details of entries from uniprot and swissprot
handle = ExPASy.get_sprot_raw('O15240')
record = SwissProt.read(handle)

print('Description : ', record.description, '\n')
print('Organism : ', record.organism, '\n')
print('Taxonomic ID : ', record.taxonomy_id, '\n')
print('Date created : ', record.created, '\n')
print('gene name : ', record.gene_name, '\n')
print('seq : ', record.sequence, '\n', 'seq length : ', record.sequence_length, '\n')
print('features : ', record.features)

# list of all authos and titles from the references
for ref in record.references:
    print('autho : ', ref.authors, '\n')

for ref in record.references:
     print('title : ', ref.title, '\n')

#motifs
instances = [Seq('TTACC'), Seq('AAACT'), Seq('CCTTA'), Seq('GGCCT'), Seq('CGATC'), Seq('AATTG'), Seq('GCGCA')]
m = motifs.create(instances)
print(m.counts, '\n')
m.weblogo('my_motif.png')
