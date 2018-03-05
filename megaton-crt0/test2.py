from bcc import BPF
BPF(text="""
#define randomized_struct_fields_start  struct {
#define randomized_struct_fields_end    };
#include <linux/sched.h>
#include <linux/fs_struct.h>
#include <linux/dcache.h>
int kprobe__sys_openat(void *ctx) {
    struct task_struct *t = (struct task_struct *)bpf_get_current_task();
    bpf_trace_printk("CWD = %s\\n", t->fs->pwd.dentry->d_name.name);
    return 0;
}""").trace_print()
