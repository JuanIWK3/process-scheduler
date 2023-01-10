from threading import Thread, Semaphore
from time import sleep

time = 0
resource = Semaphore(1)


class Process(Thread):
    def __init__(self, index):
        Thread.__init__(self)
        self.index = index

    def run(self):
        global time
        while True:
            sleep(1)

            with resource:
                print(
                    f"Process: {self.index}, entered the critical section, time: {time}")

                sleep(1)
                time += 1

                print(
                    f"Process: {self.index}, left the critical section, time: {time}")


for i in range(10):
    process = Process(i)
    process.start()

# Mutual exclusion: At least one resource must be held in a non-shareable mode; that is, only one process can use the resource at any given time.

# Cada processo adquire o recurso e quando termina de usar ele solta o rescurso
# O semáforo não permite que mais de um processo adquira o mesmo recurso

# Hold and wait: A process that is holding at least one resource is waiting to acquire additional resources held by other processes.


# No preemption: A resource can be released only voluntarily by the process holding it, after that process has completed its task.

# O processo solta o recurso voluntariamente após utilizá-lo. Não sendo interrompido

# Circular wait: A process must be waiting for a resource held by another process, which in turn is waiting for a resource held by another process, and so on. This creates a circular chain of processes, each of which is waiting for a resource held by the next process in the chain.

#
