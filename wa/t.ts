[
    ['d','date (short)'],
    ['D','date (long)'],
    ['t','time (short)'],
    ['T','time (long)'],
    ['f','full (short)'],
    ['F','full (long)'],
    ['R','relative']
].map(([s,d])=>`${d} \`${s}\`: <t:${~~(+new Date(args[0])/1000)}:${s}>`).join('\n')