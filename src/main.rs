struct Processo {
    pid: usize,
    menSize: u64,
    timeExecution: u64,
}

impl Processo {
    let mut PROXIMO_PID = 0;

    fn proximo_pid() -> usize {
        unsafe {
            let pid = Processo::PROXIMO_PID + 1;
            pid
        }
    }

    fn novo (menSize: u64, timeExecution: u64) -> Self {
        let pid = Processo::proximo_pid();
        Processo {
            pid,
            menSize,
            timeExecution,
        }
    }
}
fn main() {
    //Criando a pilha
    let mut pilha: Vec<i32> = Vec::new();
}
