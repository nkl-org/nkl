# Atomic Mass Data

Atomic mass data is extracted from neutron decay sublibrary of ENDFB, JEFF and 
JENDL nuclear data libraries.

## Atomic mass

The atomic mass for each nuclide ($m$) is computed from atomic weight ratio ($awr$)
and neutron mass ($m_n$) with following equation :

$$m = m_n \times awr$$

with

$$m_n = 1.008\ 664\ 915\ 95 \ u $$

## Format

Each atomic mass data file is a list of record with whitespace-separated fields 
and following format :

```text
..3    : atomic number (Z)
4..7   : mass number (A)
8..9   : isomeric state number (I)
10..17 : nuclide's name (XxAAAmI)
18..31 : nuclide's element name
32..34 : nuclide's element symbol
35..   : nuclide's atomic mass [u]
```

## References

### ENDFB

> Brown, D. A., et al.  
> *ENDF/B-VIII. 0: The 8th major release of the nuclear reaction data library with CIELO-project cross sections, new standards and thermal scattering data.*  
> Nuclear Data Sheets (2018).  
> Volume: 148  
> Pages: 1-142  
> DOI: https://doi.org/10.1016/j.nds.2018.02.001

### JEFF

> Plompen, A.J.M., Cabellos, O., De Saint Jean, C. et al.  
> *The joint evaluated fission and fusion nuclear data library, JEFF-3.3.*   
> The European Physical Journal (2020).  
> Volume: 56  
> Issue: 7  
> Pages: 1-108  
> DOI: https://doi.org/10.1140/epja/s10050-020-00141-9


### JENDL

> Iwamoto, O., et al.  
> *Status of JENDL*.  
> In EPJ Web of Conferences.  
> EDP Sciences (2020).  
> Volume: 239  
> Pages: 09002  
> DOI: https://doi.org/10.1051/epjconf/202023909002


