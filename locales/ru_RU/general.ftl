text-is-required = Требуется текст для подсчёта дырочек....
arg-is-required = Передайте его единственным аргументом в ковычках при запуске программы.
input-text-is-empty = В пустой строке не бывает дырочек.... 😑
holes-found =
    { $holesCount ->
        [0] В этом тексте нет дырочек 😠
        [one] В тексте найдена целая дырочка! 😮
        [few] В тексте оказалось {$holesCount} дырочки.... 😳
        *[many] В тексте оказалось {$holesCount} дырочек.... 😍
    }
uncounted-chars =
    { $uncountedCharsCount ->
        [one] Непосчитанный символ: {$uncountedCharsStr}
        *[other] Непосчитанные символы: {$uncountedCharsStr}
    }
text-label = Текст
index-title = Считаем дырочки....
count-holes-btn = Посчитать дырочки....
