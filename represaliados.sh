i=3
while true 
do
groovy https://raw.githubusercontent.com/jagedn/represaliados/master/represaliados.groovy $i
i=$((i+1))
sleep 60
done
