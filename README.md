# Yet another improved rm

## Introduce
this is a command tool that help you to delete file **except** some selectd file or folders

## Feature
- [x] use `-e` to switch mode between delete and except
- [x] Regex Support (based on walkdir)
- [ ] use recycle bin to recover files to prevent mistakes

## Example
测试待补充
```
/// rm 失败
/// rm a.txt b.py c.c 只删除文件        <Path>...
/// rm folder a.txt  删除文件和文件夹
/// rm *.txt 删除当前文件夹下的匹配文件
/// rm folder/*  删除文件夹下的东西
/// rm folder1/* folder2/* a.txt
///
/// rm folder -e a.txt  指定文件夹删除除了某些文件
/// rm -e a.txt  不指定文件夹(默认当前文件夹)
```

## Off topic
write sh is a little hard for me，and rm  !("xxx")
this small thing is
