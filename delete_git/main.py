import os


def remove_dir(path):
    if os.path.isfile(path):
        print("remove file: {}".format(path))
        os.remove(path)
    elif len(os.listdir(path)) == 0:
        print("remove dir: {}".format(path))
        os.rmdir(path)
    else:
        for dir_name in os.listdir(path):
            sub_path = os.path.join(path, dir_name)
            remove_dir(sub_path)


base_path = r"D:\Learning\codes\Rust\luogu"
for dir_name in os.listdir(base_path):
    if dir_name == ".git":
        continue
    sub_path = os.path.join(base_path, dir_name)
    if not os.path.isdir(sub_path):
        continue
    for sub_dir_name in os.listdir(sub_path):
        if sub_dir_name == ".git":
            remove_dir(os.path.join(sub_path, sub_dir_name))
        if sub_dir_name == ".gitignore":
            remove_dir(os.path.join(sub_path, sub_dir_name))
