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
    echo "Executing use case 3 - one order with minimal configuration"
    cargo run -- files/orders_files/one_order.json files/configuration_files/minimal_configuration.json

elif [ $1 = "4" ]; then
    echo "Executing use case 4 - one order with a lot of resources with basic configuration"
    cargo run -- files/orders_files/one_order_with_a_lot_of_resource.json files/configuration_files/basic_configuration.json

elif [ $1 = "5" ]; then
    echo "Executing use case 4 - 16 orders of cocoa with milk with basic configuration"
    cargo run -- files/orders_files/multiple_cocoa_with_milk_orders.json files/configuration_files/basic_configuration.json
    
elif [ $1 = "6" ]; then
    echo "Executing use case 5 - 16 orders of cocoa with milk with a lot of cocoa available"
    cargo run -- files/orders_files/multiple_cocoa_with_milk_orders.json files/configuration_files/a_lot_of_cocoa_resource_configuration.json

elif [ $1 = "7" ]; then
    echo "Executing use case 7 - 16 orders of coffee with milk with basic configuration"
    cargo run -- files/orders_files/multiple_coffee_with_milk_orders.json files/configuration_files/basic_configuration.json
    
elif [ $1 = "8" ]; then
    echo "Executing use case 8 - 16 orders of coffee with milk with a lot milk and coffee grains available"
    cargo run -- files/orders_files/multiple_coffee_with_milk_orders.json files/configuration_files/a_lot_of_grain_and_milk_resources_configuration.json

elif [ $1 = "9" ]; then
    echo "Executing use case 9"
    cargo run -- files/orders_files/a_lot_of_orders.json files/configuration_files/multiple_resources_configuration.json
    
elif [ $1 = "10" ]; then
    echo "Executing use case 10"

    
else
    echo "Invalid argument."
    exit 1
fi