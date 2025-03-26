filesystem=$1
start_dir=$2
artifact=$3
bin=/home/razmik/prj/ext4_parser/target/release/ext4_parser

if [ -z $1 ]; then
    filesystem="/dev/sdc"
fi
if [ -z $2 ]; then
    start_dir="/";
fi
if [ -z $3 ]; then
    artifact="inodes";
fi

sudo setfacl -m u:${USER}:r $filesystem
$bin $filesystem $start_dir $artifact
