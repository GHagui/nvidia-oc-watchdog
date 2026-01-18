# NVIDIA Overclock Monitor

🛡️ **Proteção automática contra reset de overclock para prevenir derretimento do conector 12VHPWR**

> 🇺🇸 [English Version](./README.md)

## 🔥 Por que este projeto existe?

O conector **12VHPWR** das placas NVIDIA RTX 4000/5000 é conhecido por ter problemas de derretimento quando submetido a picos de potência. A melhor forma de prevenir isso é através de:

1. **Undervolt com curva personalizada** no MSI Afterburner
2. **Overclock de memória** para compensar performance
3. **Monitoramento constante** para garantir que as configurações não resetem

### O Problema

O MSI Afterburner pode resetar as configurações de overclock em várias situações:
- ✗ Reinicialização do Windows
- ✗ Atualização de drivers NVIDIA
- ✗ Crash do Afterburner
- ✗ Alterações manuais acidentais
- ✗ Conflitos com outros softwares

Quando isso acontece, a GPU volta para as configurações stock, permitindo **picos de voltagem e potência perigosos** que podem derreter o conector 12VHPWR.

## ✅ A Solução

Este programa monitora continuamente o clock de memória da GPU para detectar se o perfil de overclock foi resetado. Se detectar que voltou ao stock:

1. 🔔 **Envia alerta via Telegram**
2. 🔄 **Reaplica automaticamente o perfil do Afterburner**
3. ⏱️ **Aguarda estabilização e continua monitorando**

### 🔍 Por que monitorar o clock de memória?

**O problema:** `nvidia-smi` não expõe informações sobre a curva de voltagem/frequência customizada. Não há como consultar diretamente se o undervolt está aplicado.

**A solução:** Usar o **overclock de memória como proxy de detecção**:

- Quando você aplica um perfil no Afterburner com OC de memória, o driver NVIDIA permite clocks mais altos
- `nvidia-smi` **pode** consultar o clock atual de memória via `--query-gpu=clocks.current.memory`
- Se o perfil resetar, o clock de memória volta para valores stock (mais baixos)
- **Detecção indireta:** Se o clock de memória caiu = o perfil inteiro (incluindo undervolt) foi resetado

**Exemplo prático:**
```
Com perfil aplicado:    17001 MHz (memória overclocked)
Após reset do perfil:   10501 MHz (memória stock)
```

Quando detectamos que a memória voltou ao stock, sabemos que a **curva de voltagem também resetou**, e podemos reaplicar tudo automaticamente.

**Por isso é importante ter overclock de memória no seu perfil**, mesmo que seja apenas +100 MHz - ele serve como "canário" para detectar resets.

### 🔋 Alternativa: Usar Power Limit em vez de Memory OC

**Se você não quer fazer overclock de memória**, pode usar o Power Limit como método de detecção:

- Configure o Power Limit para **99%** ou **101%** no MSI Afterburner
- Use `nvidia-smi --query-gpu=power.limit --format=csv,noheader,nounits` para consultar
- Quando o perfil resetar, o Power Limit volta para 100% (stock)

**Vantagens:**
- ✅ Não mexe com clocks de memória
- ✅ Funciona igualmente bem como "canário"
- ✅ 99% pode até melhorar temperaturas levemente

**Para implementar:** Substitua a função `get_max_mem_clock()` por uma que consulte `power.limit` e ajuste o `MEM_CLOCK_TARGET` para 99.0 ou 101.0.

## 📊 Características

- ⚡ **Extremamente leve**: Apenas ~2.1 MB de RAM
- 🔇 **Roda silenciosamente em background** (sem janela/console)
- 📱 **Alertas via Telegram** quando detecta problemas
- 🔄 **Reaplicação automática** do perfil
- ⏰ **Verificação a cada 1 hora**
- 🚀 **Zero impacto em jogos/aplicações**

## 🛠️ Instalação

### Pré-requisitos

1. **MSI Afterburner** instalado
2. **NVIDIA GPU** com drivers instalados
3. **Bot do Telegram** (para alertas)

### Configuração

#### 1. Clone o repositório
```bash
git clone https://github.com/seu-usuario/check_nvidia.git
cd check_nvidia
```

#### 2. Configure o MSI Afterburner

- Crie seu perfil com undervolt/overclock no **Perfil 1**
- Ative "Aplicar overclock na inicialização"
- Ative "Iniciar com o Windows"

#### 3. Descubra seu clock alvo

Execute no PowerShell **com o overclock aplicado**:
```powershell
nvidia-smi --query-gpu=clocks.current.memory --format=csv,noheader,nounits
```

Anote o valor (ex: 11501 MHz) e subtraia uma margem pequena (~100 MHz).

#### 4. Edite o código

Em `src/main.rs`, ajuste o valor alvo:
```rust
const MEM_CLOCK_TARGET: f64 = 11400.0; // Seu valor aqui
```

#### 5. Configure variáveis de ambiente

**Crie um bot no Telegram:**
1. Converse com [@BotFather](https://t.me/botfather)
2. Use `/newbot` e siga as instruções
3. Copie o **token** que ele fornecer

**Obtenha seu Chat ID:**
1. Converse com [@userinfobot](https://t.me/userinfobot)
2. Copie seu **ID**

**Configure no Windows (PowerShell como Administrador):**
```powershell
[System.Environment]::SetEnvironmentVariable('TELEGRAM_BOT_TOKEN', 'seu_token_aqui', 'User')
[System.Environment]::SetEnvironmentVariable('TELEGRAM_CHAT_ID', 'seu_chat_id_aqui', 'User')
```

**Reinicie o terminal** para as variáveis terem efeito.

#### 6. Compile o projeto

```bash
cargo build --release
```

O executável estará em: `target/release/check_nvidia.exe`

#### 7. Configure inicialização automática

1. Abra **Task Scheduler** (Agendador de Tarefas)
2. Clique em "Create Task" (Criar Tarefa)
3. **General**: Nome "NVIDIA Overclock Monitor"
4. **Triggers**: "At log on" + **Delay: 30 minutes**
5. **Actions**: Caminho para `check_nvidia.exe`
6. **Conditions**: Desmarque "Start only if on AC power"
7. **Settings**: Marque "Run task as soon as possible after a scheduled start is missed"

## 📱 Exemplo de Alerta

Quando detectado, você receberá no Telegram:

```
⚠️ ALERTA NVIDIA OVERCLOCK

Clock detectado: 10501 MHz
Alvo esperado: 11400 MHz

Perfil reaplicado automaticamente.
```

## 🔧 Configurações Avançadas

### Alterar intervalo de verificação

Em `src/main.rs`, linha final:
```rust
tokio::time::sleep(Duration::from_secs(3600)).await; // 3600 = 1 hora
```

Valores recomendados:
- `3600` - 1 hora (padrão, ideal para monitoramento diário)
- `1800` - 30 minutos
- `600` - 10 minutos

### Alterar perfil do Afterburner

Em `src/main.rs`:
```rust
const AB_PROFILE_ARG: &str = "-profile1"; // -profile2, -profile3, etc.
```

### Caminho customizado do Afterburner

```rust
const AB_PATH: &str = r"C:\Caminho\Customizado\MSIAfterburner.exe";
```

## 🚨 Segurança

- ✅ Credenciais do Telegram em variáveis de ambiente (não no código)
- ✅ Não expõe informações sensíveis
- ✅ Roda com permissões de usuário (não precisa admin)

## 🤝 Contribuindo

Contribuições são bem-vindas! Sinta-se à vontade para:
- 🐛 Reportar bugs
- 💡 Sugerir features
- 🔧 Enviar pull requests

## 📄 Licença

MIT License - use livremente!

## ⚠️ Aviso Legal

Este software é fornecido "como está". O uso de overclock/undervolt é por sua conta e risco. Sempre monitore temperaturas e estabilidade do sistema.

---

**Desenvolvido para proteger GPUs NVIDIA RTX 4000/5000 contra problemas do conector 12VHPWR** 🛡️🔥
