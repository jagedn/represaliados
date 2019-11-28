curl https://raw.githubusercontent.com/jagedn/represaliados/master/represaliados.groovy > represaliados.groovy
i=$1
while true 
do
echo $i
groovy represaliados.groovy $i
i=$((i+1))
sleep $(( 84 + (RANDOM % 10) + 1 ))
done
