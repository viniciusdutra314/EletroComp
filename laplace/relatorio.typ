
= Equação De Laplace

$nabla^2 V=(partial^2 V)/(partial x^2)+(partial^2 V)/(partial y^2)+(partial^2 V)/(partial z^2)=0$

Nós precisamos fazer uma discretização,
assumindo $Delta x=Delta y=Delta z=1$

$(partial^2 V)/(partial x^2)=(V(x+Delta x)-V(x)-V(x)+V(x- Delta x))/(Delta x^2)=(V(x+Delta x)-2V(x)+V(x-Delta x) )/(Delta x ^2)$

Jogando essa aproximação na equação de Laplace

$V(x,y,z)=1/6 (V(x+Delta x,y,z)+V(x-Delta x,y,z)+V(x,y+Delta y,z)+V(x,y -Delta y,z) + V(x,y,z+Delta z)+V(x,y,z-Delta z))$