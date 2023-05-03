# Trabajo práctico N°1 - CoffeeGPT

- [Trabajo práctico N°1 - CoffeeGPT](#trabajo-práctico-n1---coffeegpt)
  - [Introducción](#introducción)
  - [Diseño de la solución](#diseño-de-la-solución)
    - [Supuestos](#supuestos)
    - [Decisiones de implementación y observaciones](#decisiones-de-implementación-y-observaciones)
    - [Estructuras](#estructuras)
  - [Formatos de los archivos](#formatos-de-los-archivos)
    - [Archivo de configuracion](#archivo-de-configuracion)
    - [Archivo de órdenes](#archivo-de-órdenes)
  - [Casos de uso](#casos-de-uso)
    - [Caso de uso 1](#caso-de-uso-1)
    - [Caso de uso 2](#caso-de-uso-2)
    - [Caso de uso 3](#caso-de-uso-3)
    - [Caso de uso 4](#caso-de-uso-4)
    - [Caso de uso 5](#caso-de-uso-5)
    - [Caso de uso 6](#caso-de-uso-6)
    - [Caso de uso 7](#caso-de-uso-7)
    - [Caso de uso 8](#caso-de-uso-8)
    - [Caso de uso 9](#caso-de-uso-9)
    - [Caso de uso 10](#caso-de-uso-10)
    - [Caso de uso 11](#caso-de-uso-11)
    - [Caso de uso 12](#caso-de-uso-12)


## Introducción

El presente trabajo consiste en desarrollar una cafetera que prepara órdenes de manera concurrente.

## Diseño de la solución
--- 

### Supuestos
- No van a haber órdenes con cero ingredientes.
- Los dispensers pueden tomar los recursos de a uno a la vez. O sea, no puede un dispenser tomar al mismo tiempo dos o mas recursos de diferentes contenedores.

### Decisiones de implementación y observaciones

- Si los recursos no alcanzan para preparar una orden y ya se tomaron algunos recursos para la misma desde los contenedores, entonces esos recursos previamente tomados se desperdician. 
- Los contenedores son instancias que corren en el mismo hilo de la cafetera ya que simplemente se les va a extraer. Estas encapsulan principalmente el acceso al recurso, teniendo el monitor como campo en la estructura.
- los dispensers corren en diferentes hilos ya que preparan las ordenes de manera concurrente. 
- Si bien se analizó la posibilidad de ir a buscar otro recurso mientras el contenedor se recarga, un problema que podría llegar a generarse es que cuando se vuelva a ir a buscar el recurso (que no se obtuvo por esperar la recarga) otro dispensador haya logrado acceder al recurso y de nuevo ya no hay entonces se debe volver a esperar o ir buscar de otro recurso, pudiéndose así generar un busy wait. 
- Para implementar las estadísticas primero se analizó la posibilidad de que frenar todos los contenedores fuera mas preciso, permitiendo llevar un control mas a la par de la cantidad de órdenes realizadas junto con los recursos consumidos. Pero como las ordenes se preparan de manera concurrente, puede que se muestre una cantidad disponible de un recurso que tiene un menor nivel que la suma de recursos que requirieron las órdenes ya preparadas. Esto ocurriría porque quizas se tomo recurso para una orden que aún no fue finalizada. Por lo cual se decidió no frenar todo al mismo tiempo para minimizar los tiempos de preparación. 
- Una vez que los dispensers finalizan su ejecución, volviendo al hilo de la cafetera, la cafetera se apaga. Esto me sirvió para determinar que los niveles de los contenedores ya no iban a cambiar para asi finalizar la muestra de estadísticas por pantalla. Una vez que se apaga se retorna igualmente las estadisticas para mostrar los niveles finales.
- Cuando se tienen que imprimir que los niveles disponibles de recurso estan por debajo de X%, se decidió que se imprime una vez pero que si este se recarga se puede volver a imprimir en caso de que vuelva a bajar la cantidad de recurso por debajo de X%
- Los dispensers toman de manera secuencial los recursos y siempre en el mismo orden. Cuando toma el primer recurso (café), se empieza a llevar un registro de si este recurso, o alguno de los siguientes, esta disponible para preparar la orden con la variable `ingredient_not_available`. En caso de que no este disponible, se settea en true esta variable y la misma se utiliza para llevar un registro de si se pudo completar la orden o no y para que, en caso de que algún recurso no este disponible, no seguir tomando recursos de los siguientes contenedores (para que puedan ser aprovechados por otras órdenes). Finalizando el trabajo práctico, me di cuenta de que siempre se va a tomar café (en caso de estar disponible), no permitiendo aprovecharlo si no hay de los siguientes (cacao o espuma). Una posible mejora seria decidir el orden en que se toman los recursos de manera mas justa o de manera aleatoria, para que no se tome siempre cafe primero. 
- Si bien algunas estructuras tienen mas de una responsabilidad, se intentó refactorizar pero al tener dificultad con el lenguaje se decició priorizar finalizar con el modelo. 

### Estructuras
Explicación de las principales estructuras del modelo: 

La manera en la que se modelo el diseño es la siguiente. Se tiene una instancia de una cafetera, que corresponde a la estructura de `CoffeeMachine`, y esta al iniciar es la que inicializa los N `Dispensers`, que obtienen los recursos de los contenedores. Para implementar los contenedores, se desarrollo un `trait Container` que lo implementan 4 tipos de contenedores:

`ProviderContainer`: este tipo de contenedor es el que provee de los recursos cuando se hace necesario recargar. Se utiliza para los contenedores de granos y leche. 

`UnrechargeableContainer`: este tipo de contenedor es el que no se puede recargar. Se utiliza para el contenedor de cacao. 

`RechargeableContainer`: este tipo de contenedor es el que si puede recargarse, y se recarga mediante el llamado al controlador `ContainerRechargerController` que es el que se comunica con el contenedor que tiene el recurso necesario. Se utiliza para los contenedores de espuma de leche y de cafe molido. 

`NetworkRechargeableContainer`: este tipo de contenedor es el que se recarga de la red. No requiere de otro contenedor ni de un controlador. Se utiliza para el contenedor de agua. 

Luego, se tiene la estructura que imprime las estadisticas `StatisticsChecker`, obteniéndolas de la cafetera. 

Además, se agregaron las estructuras `Order` y `CoffeeMakerConfiguration` para poder deserializar los archivos de órdenes y configuración respectivamente. 

## Formatos de los archivos

Ambos archivos a utilizar deben estar en el formato json

### Archivo de configuracion

Consiste en un unico documento donde este tiene que tener los campos que se corresponden con la estructura `CoffeeMakerConfiguration`. 

```json
{
    "grain_capacity"                // Capacidad máxima de granos de café en el contenedor
    "ground_coffee_capacity"        // Capacidad máxima de café molido en el contenedor
    "milk_capacity"                 // Capacidad máxima de leche en el contenedor
    "milk_foam_capacity"            // Capacidad máxima de espuma en el contenedor
    "cocoa_capacity"                // Capacidad máxima de cacao en el contenedor
    "water_capacity"                // Capacidad máxima de agua en el contenedor
    "dispenser_amount"              // Cantidad de dispensers
    "coffee_ground_recharge_rate"   // Cantidad de unidades de granos de café molidos que se obtienen de una unidad de grano de café
    "milk_foam_recharge_rate"       // Cantidad de unidades de espuma que se obtienen de una unidad de leche
    "heated_water_recharge_rate"    // Cantidad de unidades de agua caliente que se obtienen de la red por milisegundo
    "amount_percentage_alert"       // Porcentaje por debajo de la capacidad máxima a la que se debe encontrar un contenedor para  alertar por consola
}
```

### Archivo de órdenes 

Consiste en una lista de órdenes, donde estas tienen que tener los campos que se corresponden con la estructura `Order`. 

```json
{
  "order_number"        // número de orden
  "coffee_amount"       // Cantidad de unidades de cafe molido
  "cocoa_amount"        // Cantidad de unidades de cacao
  "milk_foam_amount"    // Cantidad de unidades de espuma
  "water_amount"        // Cantidad de unidades de agua
}
```

## Casos de uso

Para ejecutar los casos de uso se debe ejecutar el script `use_cases.sh` de la siguiente manera:

```
sh use_cases.sh <numero-de-caso-de-uso>
```

Por ejemplo:
```
sh use_cases.sh 1
```

### Caso de uso 1

- Cantidad de órdenes a preparar: 1 
- Cantidad de órdenes preparadas: 1

Este primer caso de uso muestra el caso mas básico que es una órden con un único dispenser. Al haber recursos suficientes, se prepara con éxito. Sobran recursos y los contenedores de café molido y espuma de leche no requieren de una recarga.  

### Caso de uso 2

- Cantidad de órdenes a preparar: 4 
- Cantidad de órdenes preparadas: 4

Este caso de uso busca extender el caso uno, ya que en este se pasa a tener dos dispensers que preparan órdenes de manera concurrente. Se preparan las cuatro órdenes del archivo y también sobran recursos.

### Caso de uso 3

- Cantidad de órdenes a preparar: 1 
- Cantidad de órdenes preparadas: 1

Este caso de uso es similar al caso de uso n° 1 pero con la diferencia de que se tienen diez dispensers y sólo una orden para preparar. Se busca mostrar el comportamiento de la cafetera cuando hay menos órdenes para preparar que dispensers disponibles. Uno de los diez dispensers logra tomar la orden y la prepara. También sobran recursos. 

### Caso de uso 4

- Cantidad de órdenes a preparar: 4 
- Cantidad de órdenes preparadas: 4

Este caso de uso extiende el anterior. Se tienen cuatro ordenes a tomar por diez dispensers. Se tienen mas dispensers que órdenes para preparar pero a la vez más de un dispenser debe tomar una orden. También sobran recursos. 

### Caso de uso 5

- Cantidad de órdenes a preparar: 1 
- Cantidad de órdenes preparadas: 1

En este caso de uso se busca mostrar que se prepara una única orden y que esta agota ciertos recursos, pudiéndose de todas formas preparar. El recurso que se agota y no puede recargarse es el cacao. 

### Caso de uso 6

- Cantidad de órdenes a preparar: 1 
- Cantidad de órdenes preparadas: 0

En este caso de uso se tiene una orden que requiere de mas recursos que los diponibles en la cafetera. Es un caso borde que no es común, pero de todas maneras busca mostrar como se comporta la cafetera ante casos bordes. La orden a preparar no se completa. 

### Caso de uso 7

- Cantidad de órdenes a preparar: 16 
- Cantidad de órdenes preparadas: 10

Este caso de uso busca mostrar como se comporta la cafetera cuando las ordenes requieren sólo de un subconjunto de recursos. En este caso, el cacao se agota y como no se puede recargar no pueden finalizarse todas las órdenes. Se logran preparar diez. La espuma se agota se logra recargar y hasta termina sobrando. 

### Caso de uso 8

- Cantidad de órdenes a preparar: 16 
- Cantidad de órdenes preparadas: 16

Este caso de uso extiende el anterior, con la diferencia de que hay suficiente cacao para preparar todas las ordenes. El recurso ya no es limitante, y tampoco lo es la espuma. Entonces, se logran preparar todas las órdenes. 

### Caso de uso 9

- Cantidad de órdenes a preparar: 16 
- Cantidad de órdenes preparadas: 14

Este caso de uso busca mostrar de manera analoga al caso de 7, con la diferencia de que ambos recursos pueden recargarse. Lo interesante de este caso de uso es ver como aún pudiendo cargar café, los granos se agotan y el cafe molido no es suficiente para preparar todas las órdenes. Si se incrementara el valor de `coffee_ground_recharge_rate` en la configuracion, cada grano rendiría mas cafe molido y podría llegar a completarse todas las órdenes ya que la espuma si alcanzaba. 

### Caso de uso 10

- Cantidad de órdenes a preparar: 16 
- Cantidad de órdenes preparadas: 16

Este caso de uso es análogo al caso 8 pero con órdenes que usan espuma, agua y cafe, ya que se completan todas las órdenes pero se dispone de mayor cantidad de recursos.

**Observación:** algo que ocurrió con este caso de uso fue que en un principio tenia café molido y granos para recargar. el rate de recarga era muy alto, con lo cual debia alcanzar la cantidad de granos para poder recargar el cafe molido. Pero cuando calculaba la cantidad de granos necesarios para recargar el cafe molido, la cantidad de granos era menos de una unidad. Y era menos de una unidad porque era muy poca la cantidad de café molido que se podía cargar sin superar la cantidad maxima, pero la cantidad disponible no era suficiente para preparar las órdenes.

### Caso de uso 11

- Cantidad de órdenes a preparar: 64 
- Cantidad de órdenes preparadas: 64

En este caso de uso se muestra el resultado de preparar multiples ordenes que utilizan diferentes cantidades de cada recurso, con menos dispensadores que órdenes. Vemos que al tener una gran disponibilidad de cada recurso todas las ordenes se preparan de manera correcta.

### Caso de uso 12

- Cantidad de órdenes a preparar: 64 
- Cantidad de órdenes preparadas: 1

En este caso de uso se tienen las mismas órdenes del caso de uso 11 pero con una cantidad mínima de recursos. Una orden logra prepararse, pero esta a su vez consume la cantidad necesaria como para no poder preparar mas órdenes. 
