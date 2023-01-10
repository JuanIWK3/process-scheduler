# Relatório

O `resource` do tipo `Semaphore` permite limitar a quantidade de Threads que conseguem adquirir o `resource`. Assim não permitindo que um processo segure o recurso que outro processo precise.

## Exclusão mútua

### Um recurso está mantido em um modo não compartilhável;

## Passe e espera

### Um processo que está mantendo pelo menos um recurso está esperando para adquirir recursos adicionais mantidos por outros processos;

## Sem preempção

### Um recurso pode ser liberado apenas voluntariamente pelo processo que odetém, após esse processo ter concluído sua tarefa;

## Espera circular

### Um processo deve estar esperando por um recurso mantido por outro processo,que por sua vez está esperando por um recurso mantido por outro processo e assim por diante;
