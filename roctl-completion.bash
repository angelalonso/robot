#/usr/bin/env bash
_roctl()
{
    local cur prev

    cur=${COMP_WORDS[COMP_CWORD]}
    prev=${COMP_WORDS[COMP_CWORD-1]}

    case ${COMP_CWORD} in
        1)
            COMPREPLY=($(compgen -W "get do" -- ${cur}))
            ;;
        2)
            case ${prev} in
                get)
                    COMPREPLY=($(compgen -W "online" -- ${cur}))
                    ;;
                do)
                    COMPREPLY=($(compgen -W "check run record reset test compile gitpush" -- ${cur}))
                    ;;
            esac
            ;;
        3)
            case ${prev} in
                test)
                    COMPREPLY=($(compgen -f ${cur}))
                    ;;
            esac
            ;;
        *)
            COMPREPLY=()
            ;;
    esac
}

complete -F _roctl roctl

