#include <iostream>
#include <vector>
#include <queue>
#include <algorithm>
#include <stdio.h>

#ifdef __unix__
#include <unistd.h>
#elif defined _WIN32
#include <windows.h>
#define sleep(x) Sleep(1000 * (x))
#endif

using namespace std;

struct Process
{
    string name;
    int arrival_time;
    int burst_time;
    int time_spent;
    bool has_interruption;
    bool is_interrupted;
    int return_time;

    Process(string name, int arrival_time, int burst_time, int time_spent, bool has_interruption, bool is_interrupted, int return_time)
    {
        this->name = name;
        this->arrival_time = arrival_time;
        this->burst_time = burst_time;
        this->time_spent = time_spent;
        this->has_interruption = has_interruption;
        this->is_interrupted = is_interrupted;
        this->return_time = return_time;
    }

    int getTime() const
    {
        return burst_time;
    }
};

bool sort_by_time(Process *p1, Process *p2)
{
    return p1->getTime() < p2->getTime();
}

int find_process(vector<Process *> list, int *time_elapsed)
{
    for (int i = 0; i < list.size(); i++)
    {
        if (list[i]->arrival_time <= *time_elapsed)
        {
            return i;
        }
    }

    return 0;
}

int find_faster_process(vector<Process *> list, int *time_elapsed, Process *current_process, int time)
{
    for (int i = 0; i < list.size(); i++)
    {
        if (list[i]->arrival_time < *time_elapsed && list[i]->burst_time - list[i]->time_spent < current_process->burst_time - time)
        {
            return i;
        }
    }

    return -1;
}

int main()
{
    std::vector<Process *> list;

    list.push_back(new Process("P1", 0, 8, 0, true, false, 0));
    // list.push_back(new Process("P2", 0, 8, 0, false, false, 0));
    // list.push_back(new Process("P1", 0, 1, 0));
    // list.push_back(new Process("P2", 0, 2, 0));
    // list.push_back(new Process("P3", 0, 3, 0));
    // list.push_back(new Process("P4", 0, 4, 0));
    // list.push_back(new Process("P5", 0, 5, 0));
    // list.push_back(new Process("P6", 0, 6, 0));
    // list.push_back(new Process("P7", 16, 1, 0));
    // list.push_back(new Process("P8", 16, 2, 0));
    // list.push_back(new Process("P9", 16, 3, 0));
    // list.push_back(new Process("P10", 16, 4, 0));
    // list.push_back(new Process("P11", 27, 1, 0));
    // list.push_back(new Process("P12", 27, 2, 0));
    // list.push_back(new Process("P13", 27, 3, 0));

    int time_elapsed = 0;

    while (!list.empty())
    {
        sort(list.begin(), list.end(), sort_by_time);

        int index = find_process(list, &time_elapsed);

        Process *process = list.at(index);

        list.erase(list.begin() + index);

        if (process->is_interrupted && process->return_time > time_elapsed)
        {
            cout << "Waiting for process to return..." << endl;
            sleep(1);
            time_elapsed++;

            list.push_back(process);

            continue;
        }

        if (process->arrival_time > time_elapsed)
        {
            cout << "Process not ready" << endl;

            sleep(1);
            time_elapsed++;

            list.push_back(process);

            continue;
        }

        if (process->time_spent > 0)
        {
            cout << "\nResuming process '" << process->name << "' at " << time_elapsed << " s" << endl;
        }

        for (int time = process->time_spent; time <= process->burst_time; time++)
        {
            sort(list.begin(), list.end(), sort_by_time);

            int faster_index = find_faster_process(list, &time_elapsed, process, time);

            if (faster_index == -1)
            {
            }
            else
            {
                cout << "Found faster process: '" << faster_index << "'" << endl;

                cout << "Process '" << process->name << "' interrupted!" << endl;

                cout << "Remaining time for '" << process->name << "' is " << process->burst_time - time << " s" << endl;

                list.push_back(new Process(process->name, process->arrival_time, process->burst_time, time, process->has_interruption, process->is_interrupted, process->return_time));

                break;
            }

            if (time == 0)
            {
                cout << "\nStarting process '" << process->name << "' at " << time_elapsed << " s..." << endl;
                time_elapsed++;
                continue;
            }

            if (time == process->burst_time)
            {
                time_elapsed++;
                cout << "Process '" << process->name << "' took " << time << " s!" << endl;
                cout << "Finished process '" << process->name << "' at " << time_elapsed << " s..." << endl;
                continue;
            }

            // ! Change interruption time later
            if (process->has_interruption && time == 2)
            {

                int interruption_time = 2;

                cout << "Process taking " << time << " s" << endl;
                cout << "Interruption at 2 s for 2s" << endl;
                cout << "Process can return at " << time_elapsed + interruption_time << " s" << endl;
                cout << "Time remaining: " << process->burst_time - time << " s" << endl;

                Process *updated_process = new Process(process->name, process->arrival_time, process->burst_time, time + 1, false, true, time_elapsed + interruption_time);

                list.push_back(updated_process);

                break;
            }

            cout << "Process '" << process->name << "' taking " << time << " s" << endl;
            sleep(1);
            time_elapsed++;
        }
    }

    cout << "\nTotal time elapsed: " << time_elapsed << " s" << endl;
}
