#!/bin/bash

if [ $# -eq 0 ]; then
    echo "No arguments supplied"
    exit 1
fi

if [ $1 = "1" ]; then
    echo "Executing use case 1 - one order with basic configuration"
    cargo run -- files/orders_files/one_order.json files/configuration_files/basic_configuration.json
    
elif [ $1 = "2" ]; then
    echo "Executing use case 2 - four simple orders with basic configuration"
    cargo run -- files/orders_files/simple_orders.json files/configuration_files/basic_configuration.json
    
elif [ $1 = "3" ]; then
    echo "Executing use case 3"

elif [ $1 = "4" ]; then
    echo "Executing use case 4 - 16 orders of cocoa with milk with basic configuration"
    cargo run -- files/orders_files/multiple_cocoa_with_milk_orders.json files/configuration_files/basic_configuration.json
    
elif [ $1 = "5" ]; then
    echo "Executing use case 5 - 16 orders of cocoa with milk with a lot of cocoa resource"
    cargo run -- files/orders_files/multiple_cocoa_with_milk_orders.json files/configuration_files/a_lot_of_cocoa_resource.json
    
    
else
    echo "Invalid argument."
    exit 1
fi