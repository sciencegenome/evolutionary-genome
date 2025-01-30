# pdb-euclidean-proteome
 - rust evolutionary proteome
 - eucleadian chain comparsion for two point coordinate protein. 
 - eucledian chain coordinate comparsion for all protein atoms of the chain. 
 - knn implementation for machine learning. 

 ```
  cargo build 
 ```
 ```
 ➜  evolutionary-proteome git:(main) ✗ ./target/debug/evolutionary-pdb -h
 set of evolutionary analysis for proteome

 Usage: evolutionary-pdb <COMMAND>

 Commands:
  pdb-id
  pdb-sequence           extract the sequence of the pdf file
  eucledian-comparative  calculate the euclidean distance bettwen two chain coordinates
  eucledian-all          calculates the euclidean distance for all chain atoms
  help                   Print this message or the help of the given subcommand(s)

 Options:
  -h, --help     Print help
  -V, --version  Print version

 ```

 ```
 ➜  evolutionary-proteome git:(main) ✗ 
          ./target/debug/evolutionary-pdb eucledian-comparative sample.pdb A 10 10 N CA
 The eucledian distance between to given coordinates of the same chain is 1.4656264
 ➜  evolutionary-proteome git:(main) ✗ 
            ./target/debug/evolutionary-pdb eucledian-comparative sample.pdb A 1 1 N CA
 The eucledian distance between to given coordinates of the same chain is 1.4909409
 ```
 ```
 ➜  evolutionary-proteome git:(main) ✗ ./target/debug/evolutionary-pdb eucledian-all sample.pdb A
  The vector containing the eucleadian distance for those chain atoms are: 
   [1.4909409, 1.5401275, 1.254727, 3.479588, 1.4994173, 1.4884146 ...]

 and will write a eucledian file as  
 32.231  15.281  -13.143 32.184  14.697  -11.772 1.4909409
 32.184  14.697  -11.772 33.438  13.89   -11.387 1.5401275
 33.438  13.89   -11.387 34.102  13.07   -12.066 1.254727
 34.102  13.07   -12.066 30.797  14.065  -11.625 3.479588
 30.797  14.065  -11.625 30.976  12.589  -11.819 1.4994173
 30.976  12.589  -11.819 29.608  12.016  -11.694 1.4884146
 29.608  12.016  -11.694 28.942  12.335  -12.945 1.4526931
 28.942  12.335  -12.945 27.67   12.696  -13.05  1.3263968
 27.67   12.696  -13.05  26.901  12.777  -11.999 1.3048087
 26.901  12.777  -11.999 27.161  12.963  -14.255 2.2785378
 
```
 
 Gaurav Sablok
