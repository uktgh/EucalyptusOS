all: os.img

os.img: boot/boot.asm kernel/*.asm
	nasm -f bin boot/boot.asm -o os.img

run: os.img
	qemu-system-i386 \
		-nographic \
		-drive file=os.img,format=raw

clean:
	rm -f os.img