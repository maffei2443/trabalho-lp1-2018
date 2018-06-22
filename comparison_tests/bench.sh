#!/bin/sh
# This is a comment!
echo "Quantos testes?"
read LIM

echo "Numero da bateria de testes?"
read BAT

mkdir bateria${BAT}

echo "   "
for i in `seq 1 $LIM`
do
  cargo run > ./bateria${BAT}/log${i}
done


