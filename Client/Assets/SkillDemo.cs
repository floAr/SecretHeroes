using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class SkillDemo : MonoBehaviour
{
    public CardRenderer Renderer;
    // Start is called before the first frame update
    void Start()
    {
        StartCoroutine(DoThing());
    }

    IEnumerator DoThing()
    {
        while (true)
        {
            yield return new WaitForSeconds(1);
            var pick = Random.Range(0, 4);
            if (pick == 0)
            {
                Renderer.SkillWeapons += 1;
            }
            else if (pick == 1)
            {
                Renderer.SkillEngineering += 1;
            }
            else if (pick == 2)
            {
                Renderer.SkillBiotech += 1;
            }
            else if (pick == 3)
            {
                Renderer.SkillPsychics += 1;
            }
            Renderer.DoUpdate();

        }
    }
}
