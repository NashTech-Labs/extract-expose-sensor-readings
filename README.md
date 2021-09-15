This template pertains to read and post sensor's data to the KAA server.
<br>
Here we have two binaries present:
- **read_file_Data**
- **read_sensor_data**

# read_file_data
This binary is responsible for continuously monitoring the file and posting it to KAA server whenever read_sensor_data writes data into the file.
<br>
#### In order to run this binary, we just need to run the command:
##### Step 1: From our project directory we need to go inside the `read_file_data` folder using this command:
`cd read_file_data`
##### Step 2: We need to hit this command:
`cargo run`

# read_sensor_data
This binary is responsible for extracting magnetometer's reading and storing it to a file by using ITM.
<br>
It will write reading in the file in this format:
`"{ x: 579, y: -197 , z: -485 }"`.

This binary follows `#![no_main] #![no_std]` approach as we'll be deploying this binary in the STM32F discovery board.
<br>
#### In order to create this binary, we need to follow few steps:

Before building this project you need to solder your board. It will help in printing the data to itm terminal.
Use this [link](https://docs.rust-embedded.org/discovery/06-hello-world/index.html) to solder your f3 Board.

#### Step 1:
- Open terminal from **home directory** and execute Command

`cd /tmp && touch itm.txt`

Then

`itmdump -F -f itm.txt`

Leave this terminal running. Now in new terminal run command.

`cd /tmp && openocd -f interface/stlink-v2-1.cfg -f target/stm32f3x.cfg`

#### Step 2: Execute the Command:
`cd read_sensor_Data`

`cargo run`

Then we will be redirected to gdb terminal. Now execute command:

- `return`
- `step`
- `continue`

Now our binary will start producing sensor's data in a constant time interval. 

