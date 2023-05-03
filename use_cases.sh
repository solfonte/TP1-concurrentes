#!/bin/bash

if [ $# -eq 0 ]; then
    echo "Especifique el número de caso de uso."
    echo "sh use_cases.sh <número-de-caso-de-uso>"
    echo "Los casos de uso se encuentran especificados en el informe"
    exit 1
fi

if [ $1 = "1" ]; then
    echo "Ejecutando caso de uso n° 1 - una orden con un único dispenser"
    cargo run -- files/orders_files/one_order.json files/configuration_files/one_dispenser_configuration.json

elif [ $1 = "2" ]; then
    echo "Ejecutando caso de uso n° 2 - cuatro órdenes simples con dos dispensers"
    cargo run -- files/orders_files/simple_orders.json files/configuration_files/two_dispenser_configuration.json

elif [ $1 = "3" ]; then
    echo "Ejecutando caso de uso n° 3 - una única orden con configuración básica"
    cargo run -- files/orders_files/one_order.json files/configuration_files/basic_configuration.json
    
elif [ $1 = "4" ]; then
    echo "Ejecutando caso de uso n° 4 - cuatro órdenes simples con configuración básica"
    cargo run -- files/orders_files/simple_orders.json files/configuration_files/basic_configuration.json
    
elif [ $1 = "5" ]; then
    echo "Executing use case 5 - una única orden con configuración mínima"
    cargo run -- files/orders_files/one_order.json files/configuration_files/minimal_configuration.json

elif [ $1 = "6" ]; then
    echo "Ejecutando caso de uso n° 6 - una orden con requerimiento de muchos recursos con configuraciín básica"
    cargo run -- files/orders_files/one_order_with_a_lot_of_resource.json files/configuration_files/basic_configuration.json

elif [ $1 = "7" ]; then
    echo "Ejecutando caso de uso n° 7 - dieciséis órdenes de leche chocolatada con configuración básica"
    cargo run -- files/orders_files/multiple_cocoa_with_milk_orders.json files/configuration_files/basic_configuration.json
    
elif [ $1 = "8" ]; then
    echo "Ejecutando caso de uso n° 8 - dieciséis órdenes de leche chocolatada con mucho cacao disponible"
    cargo run -- files/orders_files/multiple_cocoa_with_milk_orders.json files/configuration_files/a_lot_of_cocoa_resource_configuration.json

elif [ $1 = "9" ]; then
    echo "Ejecutando caso de uso n° 9 - dieciséis órdenes de café con leche con configuración básica"
    cargo run -- files/orders_files/multiple_coffee_with_milk_orders.json files/configuration_files/basic_configuration.json
    
elif [ $1 = "10" ]; then
    echo "Ejecutando caso de uso n° 10 - dieciséis órdenes de café con leche con muchos granos de cafe y leche disponibles"
    cargo run -- files/orders_files/multiple_coffee_with_milk_orders.json files/configuration_files/a_lot_of_grain_and_milk_resources_configuration.json

elif [ $1 = "11" ]; then
    echo "Ejecutando caso de uso n° 11 - sesenta y cuatro órdenes con multiples recursos disponibles"
    cargo run -- files/orders_files/a_lot_of_orders.json files/configuration_files/multiple_resources_configuration.json
    
elif [ $1 = "12" ]; then
    echo "Ejecutando caso de uso n° 12 - sesenta y cuatro órdenes con configuración mínima"
    cargo run -- files/orders_files/a_lot_of_orders.json files/configuration_files/minimal_configuration.json

else
    echo "No existe ese caso de uso"
    exit 1
fi