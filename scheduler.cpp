#include <iostream>
#include <vector>
#include <queue>

#ifdef __unix__
#include <unistd.h>
#elif defined _WIN32
#include <windows.h>
#define sleep(x) Sleep(1000 * (x))
#endif

using namespace std;

struct Process
{
    int id;
    int time;
    int seconds_completed;
    bool has_interruption;
    int interruption_time;

    Process(int id, int time, bool has_interruption, int interruption_time)
    {
        this->id = id;
        this->time = time;
        this->has_interruption = has_interruption;
        this->interruption_time = interruption_time;
        this->seconds_completed = 0;
    }
};

void print_process(Process *process)
{
    cout << "\nExecuting process #" << process->id << endl;
    cout << "Process duration: " << process->time << " seconds" << endl;
    cout << "Time remaining: " << process->time - process->seconds_completed << " seconds" << endl;
}

void execute_process(Process *process, queue<Process *> *process_queue)
{
    print_process(process);

    srand(time(NULL));
    int time_of_interruption = rand() % process->time;

    for (int i = 0 + process->seconds_completed; i <= process->time; i++)
    {
        if (i == time_of_interruption && process->has_interruption)
        {
            sleep(1);
            cout << "\n\nWARN: Interruption at " << i << " seconds!" << endl;
            process->seconds_completed = i;
            process->has_interruption = false;
            process_queue->push(process);
            process_queue->pop();
            return;
        }
        else if (i == 0)
        {
            cout << "0%";
        }
        else
        {
            sleep(1);
            cout << "\r" << (double)i / process->time * 100 << "%           ";
        }
    }

    cout << endl;
    process_queue->pop();
}

void first_come_first_served(queue<Process *> process_queue)
{
    while (!process_queue.empty())
    {
        execute_process(process_queue.front(), &process_queue);
    }
}

int main()
{
    int quantity = 2;

    queue<Process *> process_queue;

    for (int i = 0; i < quantity; i++)
    {
        int process_time;
        bool has_interruption;
        int interruption_time = 0;

        cout << "time: ";
        cin >> process_time;
        cin.ignore();
        cout << "has interruption? ";
        cin >> has_interruption;
        cin.ignore();

        if (has_interruption)
        {
            srand(time(NULL));
            interruption_time = rand() % 10;
        }

        Process *process = new Process(i, process_time, has_interruption, interruption_time);
        process_queue.push(process);
    }

    first_come_first_served(process_queue);

    return 0;
}
