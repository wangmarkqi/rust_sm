from remote_run_everything import Conf, Local, Remote
def test():
    c = Conf(
        host="192.168.177.130",
        user="root",
        pwd="a",
        remote_root="/mnt/mygit/rust_sm",
        local_root="D://mygit/rust_sm",
    )
    r = Remote(c)
    r.cmd(['cargo run'])
    

test()
