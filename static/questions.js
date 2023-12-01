
const A = 1;
const B = 2;
const C = 3;
const D = 4;
const E = 5;

const question_labels = {
    "default": "Atividade 40 questões",
    //"memes": "Teste"
};

const question_pools = {
    "memes": [
        {
            "prompt": "Esse projeto de extensão afeta o grêmio?",
            "alternatives": [
                "Não afeta",
                "Afeta o grêmio",
                "Afeta só se o grêmio estiver na turma que vai participar do quiz",
                "Não afeta, pois se afetasse não estariamos fazendo esse projeto",
                "Quem é grêmio?",
            ],
            "answer": D
        },
        {
            "prompt": "Esse projeto de extensão afeta o grêmio? 2",
            "alternatives": [
                "2 Não afeta",
                "2 Afeta o grêmio",
                "2 Afeta só se o grêmio estiver na turma que vai participar do quiz",
                "2 Não afeta, pois se afetasse não estariamos fazendo esse projeto",
                "2 Quem é grêmio?",
            ],
            "answer": C
        }
    ],
    "default": [
        {
            "prompt": "Qual dos estados abaixo faz parte da região Sudeste do Brasil?",
            "alternatives": ["Tocantins", "Espírito Santo", "Paraíba", "Santa Catarina", "Paraná"],
            "answer": B,
        },
        {
            "prompt": "Qual a região brasileira que apresenta a menor densidade demográfica?",
            "alternatives": ["Norte", "Nordeste", "Sul", "Centro-Oeste", "Sudeste"],
            "answer": A,
        },
        {
            "prompt": "Qual é a maior metrópole em população da região Sudeste do Brasil?",
            "alternatives": ["Rio de Janeiro", "Brasília", "São Paulo", "Vitória", "Porto Alegre"],
            "answer": C,
        },
        {
            "prompt": "Qual bioma brasileiro, marcado pela sazonalidade das cheias, é encontrado tipicamente na região Centro-Oeste do país?",
            "alternatives": ["Araucárias", "Mata Atlântica", "Pantanal", "Mata de Cocais", "Caatinga"],
            "answer": C,
        },
        {
            "prompt": "As capitais dos estados da Região Sul do Brasil são:",
            "alternatives": ["Cuiabá, Goiânia e Campo Grande", "Manaus, Belém e Porto Velho", "Rio de Janeiro, São Paulo e Vitória", "Curitiba, Florianópolis e Porto Alegre", "Salvador, Recife e Fortaleza"],
            "answer": D,
        },
        {
            "prompt": "Cada estado brasileiro tem uma sigla. Por exemplo: a sigla do estado de São Paulo é SP. Elas nos ajudam a localizar rapidamente uma cidade no mapa do Brasil. Assim quando lemos \"Santana(AP)\" sabemos que esta se encontra no estado de (o):",
            "alternatives": ["Amazonas", "Amapá", "Acre", "Alagoas", "Não existe nenhum estado brasileiro com a sigla \"AP\""],
            "answer": B,
        },
        {
            "prompt": "Os estudos que fazem as estatísticas oficiais do Brasil são da responsabilidade de qual órgão?",
            "alternatives": ["Instituto Nacional de Geografia e Estatística (INGE).", "Instituto Brasileiro de Geografia e Estatística (IBGE).", "Instituto Brasileiro de Geografia e Estudos (IBGE).", "Instituto Nacionalista de Geografia e Estatística (INGE)."],
            "answer": B,
        },
        {
            "prompt": "A divisão regional que não obedece aos limites dos estados é chamada de:",
            "alternatives": ["Macrorregiões ou Cinco Grandes Regiões.", "Macrorregiões Geoeconômicas ou Cinco Grandes Regiões.", "Macrorregiões Geoeconômicas ou Complexos Regionais.", "Macrorregiões ou Complexos Regionais."],
            "answer": C,
        },
        {
            "prompt": "No âmbito econômico mundial, o Brasil é corretamente classificado como um país",
            "alternatives": ["emergente.", "comunista.", "desenvolvido.", "planificado.", "socialista"],
            "answer": A,
        },
        {
            "prompt": "As alternativas abaixo apresentam alguns dos mais importantes organismos e blocos econômicos mundiais. De qual deles o Brasil faz parte?",
            "alternatives": ["OCDE.", "G7.", "Asean.", "G8.", "Brics."],
            "answer": E,
        },
        {
            "prompt": "Atualmente, o cenário econômico brasileiro é marcado especialmente pela",
            "alternatives": ["importação de alimentos e minerais.", "estatização das empresas privadas.", "produção de bens de origem primária.", "exportação de eletrônicos e de chips.", "fabricação de bens de alta tecnologia."],
            "answer": C,
        },
        {
            "prompt": "As ilhas de calor representam um dos problemas ambientais urbanos. Esse fenômeno climático acontece devido:",
            "alternatives": ["ao aumento das queimadas nas zonas rurais.", "a diminuição da densidade demográfica dos centros urbanos.", "ao aumento da inversão térmica nas cidades.", "a elevação das temperaturas em algumas zonas urbanas.", "aos microclimas periféricos que afetam diretamente as cidades."],
            "answer": D,
        },
        {
            "prompt": "No contexto da globalização, uma tendência crescente é a formação de blocos econômicos regionais. Esses blocos apresentam diferentes níveis de integração. Um desses níveis é a zona de livre comércio que se caracteriza pela:",
            "alternatives": ["criação de uma moeda única a ser adotada pelos países membros.", "livre circulação de mercadorias provenientes dos países membros.", "unificação de políticas de relações internacionais entre os países membros.", "livre circulação de pessoas, serviços e capitais entre os países membros."],
            "answer": B,
        },
        {
            "prompt": "As formas de relevo resultam das grandes alterações que a superfície terrestre sofreu ao longo do tempo. São exemplos de agentes internos responsáveis por essas modificações:",
            "alternatives": ["intemperismo, vulcanismo, deposição e compactação de sedimentos.", "vulcanismo, tectonismo e abalos sísmicos.", "erosão, sedimentação e intemperismo.", "solidificação do magma, deposição e compactação de sedimentos."],
            "answer": B,
        },
        {
            "prompt": "Embora muitas vezes tempo e clima sejam utilizados como sinônimos, essas palavras possuem significados diferentes pois",
            "alternatives": ["O clima é influenciado por fatores, como precipitação e radiação solar. Já o tempo é resultado direto da influência das massas de ar.", "O tempo se refere às condições meteorológicas momentâneas e o clima representa um padrão meteorológico observado ao longo dos anos.", "O clima de uma região é resultado de fatores geográficos e o tempo é influenciado pelas condições meteorológicas.", "O tempo se refere à temperatura e a chuva em um local, já o clima reúne informações sobre pressão atmosférica, continentalidade e maritimidade."],
            "answer": B,
        },
        {
            "prompt": "Qual é o movimento da Terra que faz com que tenhamos o dia e a noite?",
            "alternatives": ["Movimento de rotação", "Movimento de translação", "Movimento de oscilação", "Movimento de rolagem", "Movimento de pulsação"],
            "answer": A,
        },
        {
            "prompt": "Essa fonte de energia muito utilizada no Brasil e no mundo é um minério fóssil que, quando processado, dá origem a vários subprodutos, como a gasolina, óleo diesel, querosene, além de gerar eletricidade nas usinas termoelétricas. A que fonte de energia refere-se o fragmento acima?",
            "alternatives": ["Gás natural", "Cana-de-açúcar", "Carvão mineral", "Petróleo", "Xisto betuminoso"],
            "answer": D,
        },
        {
            "prompt": "As fontes não renováveis podem esgotar-se totalmente em prazos variáveis (pequeno, médio e longo prazo) de acordo com a extração, consumo e disponibilidade. Das alternativas abaixo, qual delas lista apenas fontes renováveis de energia?",
            "alternatives": ["biocombustíveis, petróleo e carvão mineral.", "energia solar, energia eólica e urânio.", "urânio, gás natural e energia hidrelétrica.", "energia hidrelétrica, energia solar e biocombustíveis.", "gás natural, energia eólica e energia solar."],
            "answer": D,
        },
        {
            "prompt": "A região geográfica brasileira que é a segunda em número de habitantes é o:",
            "alternatives": ["Sudeste", "Sul", "Nordeste", "Centro-Oeste"],
            "answer": C,
        },
        {
            "prompt": "A floresta amazônica oferece serviços ambientais fundamentais, dentre os quais inclui-se a manutenção",
            "alternatives": ["da emissão de gases; ciclo vital e armazenamento de carbono.", "da fauna e flora; ciclo hidrológico e armazenamento de metano.", "do desmatamento; ciclo hidrológico e armazenamento de carbono.", "da biodiversidade; ciclo hidrológico e armazenamento de carbono.", "da biodegradação; ciclo hidrológico e armazenamento de carbono."],
            "answer": D,
        },
        {
            "prompt": "O espaço geográfico tem sido marcado pela expansão de objetos artificiais no meio. São exemplos desses objetos artificiais:",
            "alternatives": ["casas, lojas e indústrias", "florestas, lagoas e casas", "montanhas, casas e lagos", "cachoeiras, rios e lagos", "indústrias, rios e animais"],
            "answer": A,
        },
        {
            "prompt": "Quais são os dois elementos que formam o espaço geográfico?",
            "alternatives": ["Natureza e sociedade", "Natureza e espaço", "Meio ambiente e lugar", "Paisagem e espaço", "Paisagem e território"],
            "answer": A,
        },
        {
            "prompt": "A modificação do espaço geográfico provoca diversas mudanças na paisagem. Em contraposição, as paisagens que conservam as suas condições naturais são corretamente chamadas de",
            "alternatives": ["paisagem espacial", "paisagem natural.", "paisagem cultural.", "paisagem humana.", "paisagem antrópica."],
            "answer": A,
        },
        {
            "prompt": "A ciência natural que contempla os estudos da dinâmica do tempo e do clima, com enfoque na caracterização e distribuição dos tipos climáticos, é corretamente chamada de",
            "alternatives": ["Astronomia.", "Hidrologia.", "Meteorologia.", "Climatologia.", "Biologia."],
            "answer": D,
        },
        {
            "prompt": "A hidrografia brasileira é composta por rios de grande magnitude. Qual o nome do maior rio brasileiro?",
            "alternatives": ["Rio Solimões.", "Rio Paraguai.", "Rio Amazonas.", "Rio Araguaia.", "Rio Tocantins."],
            "answer": C,
        },
        {
            "prompt": "Qual o nome correto da ciência que estuda os solos?",
            "alternatives": ["Geologia.", "Pedologia.", "Geomorfologia.", "Hidrografia.", "Cartografia."],
            "answer": B,
        },
        {
            "prompt": "О quе а Віоѕfеrа?",
            "alternatives": ["Éареnаѕ о соnјuntо dаѕ саmаdаѕ аtmоѕférісаѕ.", "É а еѕfеrа dа vіdа соrrеѕроndеndо ао соnјuntо dе tоdаѕ аѕ fоrmаѕ dе vіdа dо рlаnеtа.", "É а еѕfеrа gаѕоѕа dо рlаnеtа.", "Соrrеѕроndе ао rеlеvо dо рlаnеtа"],
            "answer": B,
        },
        {
            "prompt": "Como é chamada a ciência que elabora e interpreta mapas?",
            "alternatives": ["Geografia", "Matemática", "Sociologia", "Cartografia"],
            "answer": D,
        },
        {
            "prompt": "Quаіѕ ѕãо оѕ quаtrо роntоѕ саrdеаіѕ?",
            "alternatives": ["Воrеаl, ѕеtеntrіоnаl, nоrtе е роlаr", "Lеѕtе, оеѕtе, norte е ѕul", "Nоrdеѕtе, mеrіdіоnаl, оrіеntаl е ѕul", "Nоrdеѕtе, Ѕul, осіdеntе е оrіеntе"],
            "answer": B,
        },
        {
            "prompt": "Qual o significado da palavra Geografia?",
            "alternatives": ["Estudo da vida", "História da Terra", "Descrição da paisagem", "Estudo da Terra"],
            "answer": D,
        },
        {
            "prompt": "O instrumento utilizado para medir a quantidade de chuva é:",
            "alternatives": ["anemógrafo", "heliógrafo", "termômetro", "pluviógrafo", "barômetro"],
            "answer": D,
        },
        {
            "prompt": "A Região do Brasil que possui a maior extensão territorial é:",
            "alternatives": ["Norte", "Sul", "Nordeste", "Centro-Oeste", "Sudeste"],
            "answer": A,
        },
        {
            "prompt": "Como se chama a parte explicativa do mapa, que indica as cores, os significados dos símbolos e desenhos usados no mapa?",
            "alternatives": ["legenda", "abscissa", "escala topográfica", "fonte", "orientação"],
            "answer": A,
        },
        {
            "prompt": "O título é um dos elementos obrigatórios de um mapa. A sua função é",
            "alternatives": ["estabelecer a relação entre a distância real e a distância do mapa.", "apresentar o assunto que está sendo retratado pelo documento.", "simbolizar os elementos que estão desenhados no mapa.", "informar as fontes consultadas para a confecção do documento.", "mostrar as linhas imaginárias presentes na localidade cartografada."],
            "answer": B,
        },
        {
            "prompt": "A energia solar é um tipo de energia que está em ascensão na atualidade. Um termo que caracteriza corretamente esse tipo de energia é",
            "alternatives": ["tradicional.", "primária.", "não renovável.", "poluente.", "renovável."],
            "answer": E,
        },
        {
            "prompt": "O Brasil é o maior país em extensão territorial da América do Sul. Ele faz fronteiras com 10 países da América do Sul, com exceção de",
            "alternatives": ["Bolívia e Peru.", "Guiana e Chile.", "Chile e Equador.", "Argentina e Peru.", "Equador e Guiana."],
            "answer": C,
        },
        {
            "prompt": "Todo o planeta Terra é envolvido por uma camada de ar. Essa camada gasosa que envolve a Terra é chamada:",
            "alternatives": ["hidrosfera.", "atmosfera.", "biosfera.", "litosfera."],
            "answer": 9,
        },
        {
            "prompt": "É uma representação em miniatura do planeta Terra, com seu frmato esférico, representa a superfície terrestre de maneira mais fiel que o Planisfério. Sua forma é arredondada, porém, não permite a visualização de toda a superfície ao mesmo tempo.",
            "alternatives": ["Cartas Topográficas", "Globo Terrestre", "Planisfério", "Planta", "Portulanos"],
            "answer": B,
        },
        {
            "prompt": "Região brasileira que possui o maior número de estados:",
            "alternatives": ["Região Norte", "Região Nordeste", "Região Sul", "Região Sudeste"],
            "answer": B,
        },
        {
            "prompt": "A região Norte tem o maior território do Brasil, e é composta por 7 estados. Os estados que compõem a região Norte são",
            "alternatives": ["Rio Grande do Norte, Amapá, Tocantins, São Paulo, Acre, Tocantins e Ceará.", "Ceará, Piauí, Rondônia, Amapá, Para, Amazonas e Roraima.", "Rondônia, Acre, Amazonas, Roraima, Pará, Amapá e Tocantins.", "Pará, Amazonas, Rio de Janeiro, São Paulo, Paraná, Goiás e Piauí."],
            "answer": C,
        },
    ]
};
