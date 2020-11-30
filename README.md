# Reticle (WIP)

## Install Xilinx Vivado

1. Create an account in [Xilinx](https://www.xilinx.com/registration/create-account.html)
2. Download Vivado web installer e.g., for `2020.1` it would be `Xilinx_Unified_2020.1_0602_1208_Lin64.bin`. The file name is set in both `credential.sh` and `install.sh`
3. Move `Xilinx_Unified_2020.1_0602_1208_Lin64.bin` to docker folder in this repository
4. Make sure docker image size limit is greater than 90GB, the final image is around 35GB
5. Change to docker directory `cd docker`
6. Run `./build.sh` script for setting up all Docker images
