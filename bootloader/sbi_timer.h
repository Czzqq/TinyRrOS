#ifndef SBI_TIMER_H_
#define SBI_TIMER_H_

void sbi_timer_process(void);
void clint_timer_event_start(unsigned long next_event);

#endif
